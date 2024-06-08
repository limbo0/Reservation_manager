## Implement

1. change contact details for property.
2. User privileges while creating users for the property.
3. only super users should be able to create new users for property.
4. Filter all the reservations with property id for every property user interface, since all the data for every property is stored in the same table/db.




## Usage

1. Log in to property account.
101. Can see all the reservations, but cannot edit anything unless logged in as property user. 
2. Log in using property users account.
3. Manage reservations.




1. If the login handler sends back jwt token in a response header,
How is the client handled, who just entered the cred while log in.

101. Do they have to be redirected to a new route with the headers 
changed which includes jwt token.

102. Client will decode the response header and take the auth token.
