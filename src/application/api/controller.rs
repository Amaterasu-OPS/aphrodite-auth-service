pub trait ControllerInterface {
    type Data;
    type Result;
    async fn handle(&self, data: Self::Data) -> Self::Result;
}