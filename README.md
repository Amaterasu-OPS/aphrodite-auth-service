<samp><h6 align="center">#backend, #project, #authentication</h6></samp>
# <samp align="center"><h2 align="center">Afrodite Auth service</h2></samp>

<p align="center">
  <img src="https://img.shields.io/badge/rust-22272E?&style=for-the-badge&logo=rust&logoColor=fafafa">
  <img src="https://img.shields.io/badge/docker-22272E?style=for-the-badge&logo=docker&logoColor=2496ED">
  <img src="https://img.shields.io/badge/postgres-22272E?style=for-the-badge&logo=postgresql&logoColor=4169E1">
  <img src="https://img.shields.io/badge/redis-22272E?style=for-the-badge&logo=redis&logoColor=FF4438">
  <img src="https://img.shields.io/badge/actix-22272E?style=for-the-badge&logo=actix&logoColor=fafafa">
</p>
<br/>

Afrodite is an OAuth 2.1 authentication service implementing OIDC and PKCE for secure authorization flows.
It receives PAR requests, manages user authentication, and enforces MFA and session security.
PKCE binds the authorization code to a client-generated verifier for public-client safety. 

Afrodite issues signed OIDC ID Tokens containing user identity claims.
It provides Access Tokens and Refresh Tokens for protected API access and session continuity.

## Run

Create a `.env` file in the root directory based on the `.env.example` file and adjust the environment variables as needed. In the `JWT_PRIVATE_KEY` and `JWT_PUBLIC_KEY` variables you can provide your own key pair or use the ones provided in the example.

Creating a JWT key pair can be done using the following command:

```bash
openssl genpkey -algorithm RSA -out private.pem -pkeyopt rsa_keygen_bits:4096
openssl rsa -pubout -in private.pem -out public.pem
```

Copy the contents of the `private.pem` file into the `JWT_PRIVATE_KEY` variable and the contents of the `public.pem` file into the `JWT_PUBLIC_KEY` variable.

> ⚠ Note: The `JWT_PRIVATE_KEY` and `JWT_PUBLIC_KEY` variables must be on separate lines.
> make sure to replace the `\n` characters with actual new lines

Then, you can run the service using Docker Compose:

```bash
docker compose up
```

Open [localhost:8000/api/v1/health](http://localhost:8001/api/v1/health) to access the health route.

## Authentication flow

1. **PAR Request**: Client sends a PAR request to `/api/v1/auth/par` with client credentials and requested scopes and receives a URI.
2. **Authorization Code**: Client sends the URI to the user to authorize route (`/api/v1/auth/authorize`) and Afronite redirects the user to the login page.
3. **Idp**: Client sends user credentials to the Idp route service to authenticate the user and receives an authorization code.
4. **Authorization Continue**: Client sends the authorization code to `/api/v1/auth/continue` to continue the authorization flow.
5. **Access Token**: Client sends the authorization code to `/api/v1/auth/token` to get an access token.
6. **Refresh Token**: Client sends the refresh token to `/api/v1/auth/token` to get a new access token.

## Contribute

Want to be part of this project?

Whether it’s improving documentation, fixing bugs, or adding new features — your help is always welcome.

Just fork the repo, make your changes, and open a pull request. Let’s build something great together!

## License
MIT License. See `LICENSE` file for details.
