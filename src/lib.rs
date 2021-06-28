use std::collections::HashMap;
use uuid::Uuid;
use vote::{Policy, TopicInfo};

type BordaResult = Vec<(Uuid, u32)>;

pub fn calculate(info: TopicInfo) -> BordaResult {
    let mut result: HashMap<Uuid, u32> = info
        .policies
        .iter()
        .map(|(uid, _title)| (uid.to_owned(), 0u32))
        .collect();

    let stripped = info.only_policy_voting();

    for (_, vote) in stripped {
        // first sort the votes
        vote.to_owned()
            .sort_by(|(_ida, a), (_idb, b)| a.partial_cmp(b).unwrap());

        // let mut order: Vec<&Uuid> = vote.iter().map(|(uid, _)| uid).collect();
        let order: Vec<&Uuid> = vote.iter().map(|(uid, _)| uid).collect();

        // this part considers vote trucatenation:
        // we choose Borda's original implementation, we round-down the policies that are not
        // listed in the voters vote

        /*
        let mut redacted: Vec<Policy> = info
            .policies
            .iter()
            .filter(|(uid, _)| vote.iter().any(|(v, _)| v == uid))
            .cloned()
            .collect();

        redacted.sort_by_key(|(uid, _title)| uid.to_string()); // random but determistic by uuid

        let mut not_voted: Vec<&Uuid> = redacted.iter().map(|(uid, _)| uid).collect();

        order.append(&mut not_voted);
        */

        for (i, uid) in order.iter().enumerate() {
            let point = (order.len() - i) as u32;
            let current = result.get_mut(uid).expect("unknown policy found in vote");
            *current += point;
        }
    }

    let mut sorted = result.into_iter().collect::<Vec<(Uuid, u32)>>();

    sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

    sorted
}

#[cfg(test)]
mod fptp_test {

    use super::*;

    #[test]
    fn simple() {
        let json_data = br#"{
  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "cc652ec5-0a11-48da-9189-4642473bb54e": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": [
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        1
      ]
    ]
  },
  "delegates": [
    [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "alice"
    ],
    [
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "bob"
    ],
    [
      "cc652ec5-0a11-48da-9189-4642473bb54e",
      "charlie"
    ]
  ],
  "policies": [
    [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "apples"
    ],
    [
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "bananas"
    ],
    [
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
      "orange"
    ]
  ]
}
"#;

        let info: TopicInfo = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info);
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }
    #[test]
    fn dont_include_delegates() {
        let json_data = br#"{
  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "cc652ec5-0a11-48da-9189-4642473bb54e": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ],
      [
        "046c12e1-906a-492f-8614-39dfa87d676d",
        1
      ]

    ],
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": [
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        1
      ],
      [
        "046c12e1-906a-492f-8614-39dfa87d676d",
        1
      ]

    ]
  },
  "delegates": [
    [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "alice"
    ],
    [
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "bob"
    ],
    [
      "cc652ec5-0a11-48da-9189-4642473bb54e",
      "charlie"
    ]
  ],
  "policies": [
    [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "apples"
    ],
    [
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "bananas"
    ],
    [
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
      "orange"
    ]
  ]
}
"#;

        let info: TopicInfo = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info);
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }
    #[test]
    fn multiple() {
        let json_data = br#"{
  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "046c12e1-906a-492f-8614-39dfa87d676d": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        0.8
      ],
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        0.7
      ]
    ],
    "cc652ec5-0a11-48da-9189-4642473bb54e": [
      [
        "0f18b644-3789-4194-9a98-0e08040395b7",
        1
      ]
    ],
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9": [
      [
        "a076bf38-55b3-42c0-8cd5-d89381152e10",
        1
      ]
    ]
  },
  "delegates": [
    [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "alice"
    ],
    [
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "bob"
    ],
    [
      "cc652ec5-0a11-48da-9189-4642473bb54e",
      "charlie"
    ]
  ],
  "policies": [
    [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "apples"
    ],
    [
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "bananas"
    ],
    [
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
      "orange"
    ]
  ]
}
"#;

        let info: TopicInfo = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info);
        let winner: Uuid = Uuid::parse_str("0f18b644-3789-4194-9a98-0e08040395b7").unwrap();
        assert_eq!(result[0].0, winner);
    }

    #[test]
    fn empty() {
        let json_data = br#"{

  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {},
  "delegates": [
    [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "alice"
    ],
    [
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "bob"
    ],
    [
      "cc652ec5-0a11-48da-9189-4642473bb54e",
      "charlie"
    ]
  ],
  "policies": [
    [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "apples"
    ],
    [
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "bananas"
    ],
    [
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
      "orange"
    ]
  ]
}
"#;

        let info: TopicInfo = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info);
        assert_eq!(result[0].1, result[1].1);
    }
    #[test]
    fn empty_delegates() {
        let json_data = br#"{

  "title": "topic title",
  "id": "60556c87-9af2-4e57-bf63-82ca6123082c",
  "votes": {
    "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9":[
        ["cc652ec5-0a11-48da-9189-4642473bb54e", 1.0]
    ]
  },
  "delegates": [
    [
      "2c8c6db4-ba7c-48dd-8eaa-f4ce886dc0e9",
      "alice"
    ],
    [
      "046c12e1-906a-492f-8614-39dfa87d676d",
      "bob"
    ],
    [
      "cc652ec5-0a11-48da-9189-4642473bb54e",
      "charlie"
    ]
  ],
  "policies": [
    [
      "a076bf38-55b3-42c0-8cd5-d89381152e10",
      "apples"
    ],
    [
      "0f18b644-3789-4194-9a98-0e08040395b7",
      "bananas"
    ],
    [
      "55bac309-5534-4e01-a5fc-7eae2b2d818e",
      "orange"
    ]
  ]
}
"#;

        let info: TopicInfo = serde_json::from_slice(json_data).unwrap();
        let result = calculate(info);
        assert_eq!(result[0].1, result[1].1);
    }
}
