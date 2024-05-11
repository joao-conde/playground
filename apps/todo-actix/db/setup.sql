create table if not exists todos (
  id integer primary key autoincrement, 
  title varchar(20), 
  description varchar(200)
);
