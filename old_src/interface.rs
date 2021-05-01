#[serenity::async_trait]
pub trait Interface {
    type ReceiveData;
    type SendData;

    async fn receive(&self, receive_data: Self::ReceiveData) -> anyhow::Result<()>;
    async fn send(&self, receive_data: Self::SendData) -> anyhow::Result<()>;
}

// receive() はInterfaceの外側 (Discordならserenity)
// からの要求を呑み適切な形で処理を開始させます

// 一方でsend() はInterfaceの内側 (送信処理など)
// からの要求を呑みます

// ここで注意すべきなのは, receive() やsend() は
// Transfererの機能である"転送"でしか使われないという点です
// serenityならInteraction等のpostは本来Interfaceでは想定されない処理なので,
// 各自で受送信をしてもらって構いません
// InterfaceはあくまでもTransfererとしての役回りとしての定義であることを忘れないでください
