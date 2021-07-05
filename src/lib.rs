use std::collections::HashMap;
use uuid::Uuid;
use vote::VoteData;

type BordaResult = Vec<(Uuid, u32)>;

/// I initially thought that votes casted values with 0 should be treated
/// as a white ballot (not voting), but this slight change will change the bahaviour of the
/// calculation....
/// If people do not give any number to a policy, the map will not contain that entry. In this type
/// of borda count, those entries will be 'counted down', in other terms, the borda count value
/// will be 0.
pub async fn calculate(info: VoteData) -> BordaResult {
    let mut result: HashMap<Uuid, u32> = info
        .policies
        .iter()
        .map(|uid| (uid.to_owned(), 0u32))
        .collect();

    let stripped = info.only_policy_voting();

    let vote_vec = stripped
        .iter()
        .map(|(_, vote)| {
            vote.iter()
                .map(|(u, v)| (u.to_owned(), *v))
                .collect::<Vec<(Uuid, f64)>>()
        })
        .collect::<Vec<Vec<(Uuid, f64)>>>();

    // TODO: I can parrallize this, sorting and putting point can be done asyncly
    for vote in vote_vec {
        // first sort the votes
        let mut sorted = vote.to_owned();
        sorted.sort_by(|(_ida, a), (_idb, b)| b.partial_cmp(a).unwrap());

        // let mut order: Vec<&Uuid> = vote.iter().map(|(uid, _)| uid).collect();
        let order: Vec<&Uuid> = sorted.iter().map(|(uid, _)| uid).collect();

        println!("{:?}", &order);

        for (i, uid) in order.iter().enumerate() {
            let point = (info.policies.len() - i) as u32;
            let current = result.get_mut(uid).expect("unknown policy found in vote");
            *current += point;
        }
    }

    let mut sorted = result.into_iter().collect::<Vec<(Uuid, u32)>>();

    sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

    sorted
}

#[cfg(test)]
mod borda_test {

    use super::*;

    #[actix_rt::test]
    async fn simple() {
        let json_data = br#"{
    "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": {
        "0f18b644-3789-4194-9a98-0e08040395b7":1
    },
    "cc652ec5-0a11-48da-9189-4642473bb54e": {
    "0f18b644-3789-4194-9a98-0e08040395b7": 1.0
      },
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": {
        "a076bf38-55b3-42c0-8cd5-d89381152e10":1
      }
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e"
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e"
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info).await;
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }
    #[actix_rt::test]
    async fn dont_include_delegates() {
        let json_data = br#"{
  "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": {
        "0f18b644-3789-4194-9a98-0e08040395b7":1
    },
    "cc652ec5-0a11-48da-9189-4642473bb54e": {
        "0f18b644-3789-4194-9a98-0e08040395b7":1,
        "046c12e1-906a-492f-8614-39dfa87d676d":1
    },
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": {
        "a076bf38-55b3-42c0-8cd5-d89381152e10":1,
        "046c12e1-906a-492f-8614-39dfa87d676d":1
    }
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e"
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e"
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info).await;
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }
    #[actix_rt::test]
    async fn multiple() {
        let json_data = br#"{
   "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": {
        "0f18b644-3789-4194-9a98-0e08040395b7":0.8,
        "a076bf38-55b3-42c0-8cd5-d89381152e10":0.7
    },
    "cc652ec5-0a11-48da-9189-4642473bb54e": {
        "0f18b644-3789-4194-9a98-0e08040395b7": 1
    },
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": {
        "a076bf38-55b3-42c0-8cd5-d89381152e10": 1
    }
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e"
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e"
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info).await;
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }

    #[actix_rt::test]
    async fn empty() {
        let json_data = br#"{

  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {},
  "delegates":[
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e"
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e"
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info).await;
        assert_eq!(result[0].1, result[1].1);
    }
    #[actix_rt::test]
    async fn empty_delegates() {
        let json_data = br#"{
  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9":{
        "cc652ec5-0a11-48da-9189-4642473bb54e":1.0
    }
  },
  "delegates": [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "cc652ec5-0a11-48da-9189-4642473bb54e"
  ],
  "policies": [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "55bac309-5534-4e01-a5fc-7eae2b2d818e"
  ]
}
"#;

        let info: VoteData = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info).await;
        assert_eq!(result[0].1, result[1].1);
    }
}
