use pubsub_rs::{Pubsub, PubsubError};

#[tokio::test]
async fn subscriber_receives_error_when_pubsub_dropped() {
    let pubsub: Pubsub<&str, String> = Pubsub::new();
    let subscriber = pubsub.subscribe(vec!["a", "b"]).await;
    drop(pubsub);

    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        let Err(e) = subscriber.recv().await else {
            panic!("should get error");
        };
        assert_eq!(e, PubsubError);
        tx.send(()).unwrap();
    });
    rx.await.unwrap();
}

#[tokio::test]
async fn subscriber_receives_error_when_pubsub_dropped_in_spawned_task() {
    let pubsub: Pubsub<&str, String> = Pubsub::new();
    let subscriber = pubsub.subscribe(vec!["a", "b"]).await;

    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        tokio::spawn(async move {
            drop(pubsub);
        });

        let Err(e) = subscriber.recv().await else {
            panic!("should get error");
        };
        assert_eq!(e, PubsubError);
        tx.send(()).unwrap();
    });
    rx.await.unwrap();
}

#[tokio::test]
async fn pubsub_clone_allows_publish_after_original_dropped() {
    let pubsub: Pubsub<&str, String> = Pubsub::new();
    let pubsub_clone = pubsub.clone();
    let subscriber = pubsub.subscribe(vec!["a", "b"]).await;

    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        tokio::spawn(async move {
            drop(pubsub);
            tokio::spawn(async move {
                pubsub_clone.publish("a", "test".to_owned()).await;
            });
        });

        let Ok((topic, payload)) = subscriber.recv().await else {
            panic!("should not get error");
        };

        assert_eq!(topic, "a");
        assert_eq!(payload, "test");

        tx.send(()).unwrap();
    });

    rx.await.unwrap();
}
