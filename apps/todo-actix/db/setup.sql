create table if not exists todos (
  id integer primary key autoincrement not null, 
  title varchar(20) not null,
  description varchar(200) not null
);
