
With the time limit of the task i've used rust's standard async and kept it simple

Assuming that the only time I edit a newly created user during initialisation is if it's a deposit. 

Every other transaction type I just create a new default client. Even with chargebacks, dispute and resolve transaction types

Hashmap has been opted as the memory storage for the constant time lookup especially when the collections get large


