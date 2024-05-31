use jsonwebtoken::{decode_header, Algorithm, Validation};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct JWTClaims {
    aud: String,
    sub: String,
    exp: u64, // experiation
}

pub fn validate(key: &[u8; 64], token: String) {
    // let key = b"lIoZjhkjsQYpiU08LFGHaJUrddNP1g51dViYZhUuzKF4an4Qkz9MNfvIjiIT5Ude";
    // let my_claims = JWTClaims {
    //     aud: "https://crm.yayleads.mx".to_string(),
    //     sub: "yayleads".to_string(),
    //     exp: 1716922448352,
    // };

    // let token = match encode(
    //     &Header::default(),
    //     &my_claims,
    //     &EncodingKey::from_secret(key),
    // ) {
    //     Ok(t) => t,
    //     Err(e) => format!("ERROR :: {:?}", e),
    // };

    error!("TOKEN:: {:?}", token);
    let mut validation = Validation::new(Algorithm::RS256);
    // validation.sub = Some("yayleads".to_string());
    validation.set_issuer(&["https://dev-zv75zriia3jcgnej.us.auth0.com/"]);
    validation.set_audience(&["https://crm.yayleads.mx"]);
    validation.set_required_spec_claims(&["_permissions"]);
    error!("VALIDATION:: {:?}", validation);

    // let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
    //     Ok(c) => c,
    //     Err(err) => match *err.kind() {
    //         ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
    //         ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
    //         _ => panic!("Some other errors {err:?}"),
    //     },
    // };
    let token_data = decode_header(&token);
    match token_data {
        Ok(data) => error!("TOKEN_DATA:: {:?}", data),
        Err(e) => error!("TOKEN_DATA ERROR:: {:?}", e),
    }

    // println!("{:?}", token_data.claims);
    // println!("{:?}", token_data.header);
}
