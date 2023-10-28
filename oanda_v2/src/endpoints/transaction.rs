use crate :: client :: Client ; struct Transaction < 'a > { client : & 'a Client , } impl < 'a > Transaction < 'a > { # [doc = " Get a list of Transactions pages that satisfy a time-based Transaction query."] pub async fn transactions (& self , authorization : String , accept_datetime_format : AcceptDatetimeFormat , account_id : AccountId , from : DateTime , to : DateTime , page_size : Integer , type : ListOf) -> Result < () > { let url = self . client . url ("/v3/accounts/{accountID}/transactions") ; self . client . get (url) . header ("Authorization" , authorization) . header ("Accept-Datetime-Format" , accept_datetime_format) ; } # [doc = " Get the details of a single Account Transaction."] pub async fn get (& self , authorization : String , accept_datetime_format : AcceptDatetimeFormat , account_id : AccountId , transaction_id : TransactionId) -> Result < () > { let url = self . client . url ("/v3/accounts/{accountID}/transactions/{transactionID}") ; self . client . get (url) . header ("Authorization" , authorization) . header ("Accept-Datetime-Format" , accept_datetime_format) ; } # [doc = " Get a range of Transactions for an Account based on the Transaction IDs."] pub async fn idrange (& self , authorization : String , accept_datetime_format : AcceptDatetimeFormat , account_id : AccountId , from : TransactionId , to : TransactionId , type : ListOf) -> Result < () > { let url = self . client . url ("/v3/accounts/{accountID}/transactions/idrange") ; self . client . get (url) . header ("Authorization" , authorization) . header ("Accept-Datetime-Format" , accept_datetime_format) ; } # [doc = " Get a range of Transactions for an Account starting at (but not including) a provided Transaction ID."] pub async fn sinceid (& self , authorization : String , accept_datetime_format : AcceptDatetimeFormat , account_id : AccountId , id : TransactionId , type : ListOf) -> Result < () > { let url = self . client . url ("/v3/accounts/{accountID}/transactions/sinceid") ; self . client . get (url) . header ("Authorization" , authorization) . header ("Accept-Datetime-Format" , accept_datetime_format) ; } # [doc = " Get a stream of Transactions for an Account starting from when the request is made."] pub async fn stream (& self , authorization : String , account_id : AccountId) -> Result < () > { let url = self . client . url ("/v3/accounts/{accountID}/transactions/stream") ; self . client . get (url) . header ("Authorization" , authorization) ; } }