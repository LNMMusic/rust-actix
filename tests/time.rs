use std::time::{SystemTime, UNIX_EPOCH, Duration};


#[test]
fn testing() {
    let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    
    let exp_1 = (iat + Duration::from_secs(30)).as_secs() as usize;
    let exp_2 = iat.as_secs() as usize + 30;

    assert_eq!(exp_1, exp_2)
}