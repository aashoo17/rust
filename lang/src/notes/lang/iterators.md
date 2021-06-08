## Iterators

can be used on variable containing group of data such as Vec,Array etc

what does calling iter(),iter_mut() and into_iter() do
call them on a vector and see result
this result after iter,iter_mut and into_iter call is called as is something called iterator 
which can be looped over with for... in or advanced calling next() method.
thats why iterator is called lazy as nothing happens until next() is called
or called by some other on your behalf such as for..in,sum() etc

## consuming iterator
that means iterator itself is piece of data in memory nothing happens 
unless next() is called manually or use for..in which will call next()
automatically.
who will call next() called consuming iterator


## Iterator adapters
after calling these methods on iterator they can modify the iterator change the
data make them bigger or smaller etc

again it will just modify the iterator nothing happens unless next() is called

## creating own iterator with Iterator trait

