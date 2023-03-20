/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */
 use aws_sdk_sesv2::model::{Body, Content, Destination, EmailContent, Message};
 use aws_sdk_sesv2::{Client, Error, Region, PKG_VERSION};
 use structopt::StructOpt;
 
 #[derive(Debug, StructOpt)]
pub struct Opt {
     #[structopt(short, long)]
     pub contact_list: String,
 
     #[structopt(short, long)]
     pub region: Option<String>,
 
     #[structopt(short, long)]
     pub from_address: String,
 
     /// The message of the email.
     #[structopt(short, long)]
     pub message: String,
 
     #[structopt(short, long)]
     pub subject: String,
 
     /// Whether to display additional information.
     #[structopt(short, long)]
     pub verbose: bool,
 }
 
 // Sends a message to all members of the contact list.
 // snippet-start:[ses.rust.send-email]
 pub async fn send_message(
     client: &Client,
     list: &str,
     from: &str,
     subject: &str,
     message: &str,
 ) -> Result<(), Error> {
     // Get list of email addresses from contact list.
     let resp = client
         .list_contacts()
         .contact_list_name(list)
         .send()
         .await?;
 
     let contacts = resp.contacts().unwrap_or_default();
 
     let cs: String = contacts
         .iter()
         .map(|i| i.email_address().unwrap_or_default())
         .collect();
 
     let dest = Destination::builder().to_addresses(cs).build();
     let subject_content = Content::builder().data(subject).charset("UTF-8").build();
     let body_content = Content::builder().data(message).charset("UTF-8").build();
     let body = Body::builder().text(body_content).build();
 
     let msg = Message::builder()
         .subject(subject_content)
         .body(body)
         .build();
 
     let email_content = EmailContent::builder().simple(msg).build();
 
     client
         .send_email()
         .from_email_address(from)
         .destination(dest)
         .content(email_content)
         .send()
         .await?;
 
     println!("Email sent to list");
 
     Ok(())
 } 