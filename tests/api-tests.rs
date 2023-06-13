use anyhow::Result;
use serde_json::json;


#[tokio::test]
async fn api_tests() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let user_add_test = hc.do_post(
        "/api/user/add",
        json!({
            "first_name": "Jon",
            "surname": "Butterworth",
            "username": "jonbut34",
            "email": "jon@jonbut.com",
            "password": "123"
        }));
    user_add_test.await?.print().await?;

    // let user_delete_test = hc.do_delete(
    //     "/api/user/delete/jon@jonbut.com",
    //     );
    // user_delete_test.await?.print().await?;

    // let test_get_key = hc.do_get("/api/crypt/new-key");
    // test_get_key.await?.print().await?;

    Ok(())

}