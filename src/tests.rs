use std::net::Ipv4Addr;
use crate::request;

#[tokio::test]
async fn skyblock() {
    let server = request(Ipv4Addr::new(202, 165, 126, 56), None).await.unwrap();
    assert_eq!(server.ip, "202.165.126.56");
    assert_eq!(server.port, 25565);
    assert!(server.debug.is_some());
}