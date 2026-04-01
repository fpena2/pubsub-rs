use pubsub_rs::Pubsub;

#[tokio::test]
async fn subscriber_can_be_moved_to_different_thread() {
    let pubsub = Pubsub::new();
    let subscriber = pubsub.subscribe(vec!["a", "b"]).await;

    pubsub.publish("a", "msg1".to_owned()).await;

    tokio::spawn(async move {
        let _ = subscriber.recv().await.unwrap();
    });
}
