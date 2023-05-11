use tokio::{io, sync::mpsc};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = mpsc::channel(100);
    
    for i in 0..10 {
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(format!("data: {}", i)).await.unwrap();
        });
    }

    drop(tx);

    while let Some(res) = rx.recv().await {
        println!("{}", res);
    }

    Ok(())
}
