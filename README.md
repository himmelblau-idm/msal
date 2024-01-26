MSAL
====

The purpose of this project is to implement MSAL for Rust, based on the specifications found in the Microsoft API Reference for [ClientApplication Class](https://learn.microsoft.com/en-us/python/api/msal/msal.application.clientapplication?view=msal-py-latest) and [PublicClientApplication Class](https://learn.microsoft.com/en-us/python/api/msal/msal.application.publicclientapplication?view=msal-py-latest). These are Python references which will be mimicked in Rust here.

> **_NOTE:_**  Implementing the [ConfidentialClientApplication Class](https://learn.microsoft.com/en-us/python/api/msal/msal.application.confidentialclientapplication?view=msal-py-latest) is not currently a target for this project. If you are interested in volunteering to implement the ConfidentialClientApplication Class, please contact the maintainer.

The project also implements the [MS-DRS] protocol, which is undocumented by
microsoft. A [protocol specification](https://github.com/himmelblau-idm/aad-join-spec/releases/latest)
is in progress as part of the himmelblau project.

In addition to the ClientApplication Class and [MS-DRS] implementations, this project implements [MS-OAPXBC] sections [3.1.5.1.2 Request for Primary Refresh Token](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oapxbc/d32d5cd0-05d4-4ec2-8bcc-ac29ce711c23) and [3.1.5.1.3 Exchange Primary Refresh Token for Access Token](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oapxbc/06e2bf0d-8cea-4b11-ad78-d212330ebda9). These are not implemented in Microsoft's MSAL libraries, but are possible when authenticating from an enrolled device.

How do I use this library?
--------------------------

Import the module into your project, then include the PublicClientApplication:

```Rust
use msal::PublicClientApplication;
```

Create an instance of the PublicClientApplication, then authenticate:

```Rust
let authority = format!("https://login.microsoftonline.com/{}", tenant_id);
let app = PublicClientApplication::new(client_id, Some(&authority));
let scope = vec![];
let token = app.acquire_token_by_username_password(username, password, scope).await?;
```

You can obtain your `client_id` and `tenant_id` from the Azure portal.

You can perform a silent auth using a previously obtained refresh token:

```Rust
let token = app.acquire_token_silent(scope, &token.refresh_token).await?;
```

Or finally, you can perform a Device Authorization Grant:

```Rust
let flow = app.initiate_device_flow(scope).await?;

// Prompt the user with the message found in flow.message

let token = app.acquire_token_by_device_flow(flow).await?;
```

If msal is built with the `broker` feature, you can enroll the device, then request an authentication token:

```Rust
use openssl::pkey::PKey;
use openssl::rsa::Rsa;

// First create an RSA2048 private key for enrolling the device.
let id_key = Rsa::generate(2048).expect("Failed generating 2048 bit RSA key");

let app = BrokerClientApplication::new(Some(&authority));

// Obtain a token for authentication. If authenticating here without MFA, the PRT and
// user token will not have the mfa claim. Use initiate_device_flow_for_device_enrollment()
// and acquire_token_by_device_flow() to authenticate with the
// mfa claim.
let token = app.acquire_token_by_username_password_for_device_enrollment(username, password).await?;
// Use the RSA2048 private key for enrollment. This private key now represents
// your device during authentication.
let (cert, device_id) = app.enroll_device(&token, domain, &id_key).await?;

// Request an authentication token
let token = app.acquire_token_by_username_password(username, password, scope, &id_key, &cert).await?;
```
