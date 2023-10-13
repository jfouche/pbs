pub static BASE_API_URL: &str = "http://localhost:3030/";

// pub async fn get_stories(count: usize) -> Result<Vec<StoryItem>, reqwest::Error> {
//     let url = format!("{}topstories.json", BASE_API_URL);
//     let stories_ids = &reqwest::get(&url).await?.json::<Vec<i64>>().await?[..count];

//     let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
//         .iter()
//         .map(|&story_id| get_story_preview(story_id));
//     Ok(join_all(story_futures)
//         .await
//         .into_iter()
//         .filter_map(|story| story.ok())
//         .collect())
// }
