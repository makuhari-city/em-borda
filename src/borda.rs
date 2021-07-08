use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use vote::{AggregationRule, VoteData};

pub struct RankChoiceVoting {}

#[async_trait]
impl AggregationRule for RankChoiceVoting {
    /// Rank Choice Voting
    /// we first check the popularity by order,
    /// if we see a candidate that exceeds majority
    /// we select that, if not, we reject the least popular
    /// we do this until we find a candidate that exceeds the majority.
    /// we first need to have a vec that shows the order of each
    /// voters preference
    async fn calculate(data: VoteData) -> Value {

        let votes = data.only_policy_voting();
        
        let prefs: Vec<Vec<Uuid>> = data.delegates.iter().map(|(_, vote)|{
        let mut ordered:Vec<(Uuid, f64)> = vote.iter().collect;
        ordered.sort_by(|(_, a), (_,b)|b.cmp(a))
            ordered.iter().map(|(id, _)|id.to_owned()).collect::<Vec<Uuid>>()
        }).collect();

        let passed = 
        let mut rejected: Vec<Uuid> = Vec::new();

    }


    fn one_pass(prefs: Vec<Vec<Uuid>>, )



        let mut sorted = result.into_iter().collect::<Vec<(Uuid, u32)>>();

        sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

        serde_json::to_value(sorted).expected("this should be deserializeable")
    }
}


