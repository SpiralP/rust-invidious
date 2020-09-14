use invidious::api::videos;

#[tokio::test]
async fn test_videos() {
  let r = videos::request("jb7cTUwPG-A", videos::Parameters::default())
    .await
    .unwrap();

  assert_eq!(r.title, "Justice - Cross (Full Album)");
}
