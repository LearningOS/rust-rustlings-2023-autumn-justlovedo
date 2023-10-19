use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        // Try to get a mutable reference to the first team's Team struct in the scores HashMap
        scores
            .entry(team_1_name.clone())
            // If the Team struct exists, modify it by incrementing the goals_scored and goals_conceded fields
            .and_modify(|team| {
                team.goals_scored += team_1_score;
                team.goals_conceded += team_2_score;
            })
            // If the Team struct does not exist, insert a new one with the initial scores
            .or_insert(Team {
                name: team_1_name,
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            });

        // Repeat the same process for the second team
        scores
            .entry(team_2_name.clone())
            .and_modify(|team| {
                team.goals_scored += team_2_score;
                team.goals_conceded += team_1_score;
            })
            .or_insert(Team {
                name: team_2_name,
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            });
    }
    scores
}