// use mpsc and oneshots together as spmc
use anyhow::{Context, Result};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
struct SpmcSender<T> {
    r_request: mpsc::Receiver<oneshot::Sender<T>>,
}

#[derive(Clone, Debug)]
struct SpmcReceiver<T> {
    s_request: mpsc::Sender<oneshot::Sender<T>>,
}

fn spmc_channel<T>() -> (SpmcReceiver<T>, SpmcSender<T>) {
    let (s_request, r_request) = mpsc::channel::<oneshot::Sender<T>>(1);
    (SpmcReceiver { s_request }, SpmcSender { r_request })
}

impl<T: 'static> SpmcReceiver<T> {
    async fn recv(&self) -> Result<T>
    where
        T: Copy + Send + Sync,
    {
        let (s_resp, r_resp) = tokio::sync::oneshot::channel();
        self.s_request
            .send(s_resp)
            .await
            .context("sending request to producer")?;
        Ok(r_resp.await.context("awaiting response from producer")?)
    }
}

impl<T: 'static> SpmcSender<T>
where
    T: Sync + Send + Copy + std::fmt::Debug,
{
    async fn send(&mut self, value: T) {
        if let Some(s_resp) = self.r_request.recv().await {
            s_resp.send(value).unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let (r_spmc, mut s_spmc) = spmc_channel::<usize>();
    let _producer = tokio::spawn(async move {
        let mut n = 1;
        loop {
            s_spmc.send(n).await;
            n += 1;
        }
    });
    let a = {
        let r_spmc = r_spmc.clone();
        tokio::spawn(async move {
            for _ in 0..3 {
                println!("A received {:?} from producer", r_spmc.recv().await);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        })
    };
    let b = {
        tokio::spawn(async move {
            for _ in 0..3 {
                println!("B received {:?} from producer", r_spmc.recv().await);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        })
    };
    println!("joining: {:?}", tokio::join!(a, b));
}
