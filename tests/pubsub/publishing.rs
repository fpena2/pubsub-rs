use pubsub_rs::Pubsub;

#[tokio::test]
async fn can_publish_to_different_topics() {
    let pubsub = Pubsub::new();
    let subscriber = pubsub.subscribe(vec!["a", "b"]).await;

    pubsub.publish("a", "msg1".to_owned()).await;
    pubsub.publish("b", "msg2".to_owned()).await;

    {
        let (topic, msg) = subscriber.recv().await.unwrap();
        assert_eq!(topic, "a");
        assert_eq!(msg, "msg1");
    }
    {
        let (topic, msg) = subscriber.recv().await.unwrap();
        assert_eq!(topic, "b");
        assert_eq!(msg, "msg2");
    }
}

#[tokio::test]
async fn publish_can_be_called_from_different_thread() {
    let pubsub = Pubsub::new();
    let pubsub_handle = pubsub.clone();
    let subscriber = pubsub.subscribe(vec!["a", "b"]).await;

    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        pubsub_handle.publish("a", "msg1".to_owned()).await;
        tx.send(()).unwrap();
    });

    rx.await.unwrap();

    let (topic, msg) = subscriber.recv().await.unwrap();
    assert_eq!(topic, "a");
    assert_eq!(msg, "msg1");
}
