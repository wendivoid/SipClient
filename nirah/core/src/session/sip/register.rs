use crate::prelude::*;
use nirah_sip::parse_message;

impl SipSessionProvider {
    pub(crate) async fn register(&mut self) -> NirahResult<()> {
        let client = unwrap_mut_or_else_not_connected!(self, client, "Client not connected");
        let acc = { unwrap_or_else_not_connected!(self, acc, "Local Uri has not been set").clone() };
        let addr = acc.get_socket_address();
        let req = client.get_register_request()?;
        let data = format!("{}", req);
        if let Some(socket) = &mut self.socket {
            socket.send_to(data.as_ref(), &addr).await?;
        }
        let response = self.read_future().await?;
        let (_, challenge_response) = parse_message(response.as_ref())?;
        if let Some(200) = challenge_response.status_code() {
            self.reg_timeout = Some(tokio::clock::now());
            return Ok(());
        }
        let data = if let Some(client) = &mut self.client {

            client.set_register_challenge(challenge_response)?;
            let req = client.get_register_request()?;
            format!("{}", req)
        } else {
            return Ok(());
        };
        if let Some(socket) = &mut self.socket {
            socket.send_to(data.as_ref(), &addr).await?;
        }
        let response = self.read_future().await?;
        let (_, final_response) = parse_message(response.as_ref())?;
        if let Some(200) = final_response.status_code() {
            self.reg_timeout = Some(tokio::clock::now());
            Ok(())
        } else {
            trace!("Failed Sip Message: {:?}", response);
            Err(NirahError::SipRegistrationFailed(acc.id))
        }

    }

}
