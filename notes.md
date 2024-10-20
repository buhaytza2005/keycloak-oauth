The example of GraphClient

GraphClient
    - Client

Client:
    - client_application
    - inner


The Client has the inner client (in graph-rs it's a reqwest client for the http but I can use BasicClient)
The client_application is based on a trait `ClientApplication` which is implemented on types that can retrieve a token. This can be a token where an implementation of `get_token()` can just return the token, or an 



KeycloakClient
    - Client

Client:
    - client_application
    - inner

