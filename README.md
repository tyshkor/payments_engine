# Payments Engine

### Design decisions

I've decided to use Vec<Option<AccountEnriched>> instead of HashMap<AccountEnriched> or BTreeMap<AccountEnriched> as I assumed high loads on the application. Therefore, in case I've used any kind of hashmaps I'd get a performance hit when the maps are going to be resized.

I've decided to use BTreeMap for transactions field in the AccountEnriched struct, as I assumed, that the number of transactions will be very limited for each Account, compared to the total number of transactions.
Also, I've used BTreeMap instead of HashMap, as I'd get automatic ordering by insert. This is very important as non-deposit and non-withdraw transaction don't have unique ids.

I've decided to use rayon crate to do the work in parallel, as transactions don't have any coupling. But I could do this only after dividing them into chunks that correspond to each account. This is a very low-cost job, compared to actual business logic.
Could we use channels, to make it even faster? Theoretically, yes, but in practice that would mean that we would need a lot of extra instruments to deals with the problem of not having enough OS-threads to deal with all of the channels simultaneously. And we still needed to have the same data structure for transactions (BTreeMap) to be able to check in case of dispute if the transactions needed actually existed. By the way we would do the business logic exactly after sending the message to the channel. That would mean actual performance hit, as in case of my design OS-thread will do all the work for one Account, therefore having just one call, while channel-based design would go back and forth between threads.
That's why I'd better divide the file into chunks of manageable size and process them consecutively, rather than process them in a stream manner.

Talking about multiple csv files to a web server, I'd say that that is an idea that needs significant clarification.
The problem is, that the order means a lot, therefore the order of files mean a lot.
Moreover, how can we guarantee, that we won't get some transactions in one file, that should be in the middle of the transactions of the other file.
These facts could easily ruin the business logic stated in the task.

### Error handling

Basically the task explicitly states to ignore any meaningful errors. I assume that is done to save the development time.

### Testing

I've tested the app using sample data situated in test_data directory.
I've tested only business logic, because writing csv out, dealing with CLI arguments was already extensively done by respective teams.
