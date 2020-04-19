use invidious::api::videos;

#[tokio::test]
async fn test_videos() {
  let r = videos::request("gQngg8iQipk", videos::Parameters::default())
    .await
    .unwrap();

  assert_eq!(r.title, "Ferris' Sand Canyon");
}
