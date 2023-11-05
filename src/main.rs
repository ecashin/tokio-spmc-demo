// use mpsc and oneshots together as spmc
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (s_req, mut r_req) = mpsc::channel::<oneshot::Sender<usize>>(1);

    let _producer = tokio::spawn(async move {
        let mut n = 1;
        while let Some(s_resp) = r_req.recv().await {
            s_resp.send(n).unwrap();
            n += 1;
        }
    });
    let a = {
        let s_req = s_req.clone();
        tokio::spawn(async move {
            for _ in 0..3 {
                let (s_resp, r_resp) = tokio::sync::oneshot::channel();
                s_req.send(s_resp).await.unwrap();
                println!("A received {} from producer", r_resp.await.unwrap());
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        })
    };
    let b = tokio::spawn(async move {
        for _ in 0..3 {
            let (s_resp, r_resp) = tokio::sync::oneshot::channel();
            s_req.send(s_resp).await.unwrap();
            println!("B received {} from producer", r_resp.await.unwrap());
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });
    println!("joining: {:?}", tokio::join!(a, b));
}
