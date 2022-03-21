drop table if exists course;


create table course
(
  id serial primary key,
  teacher_id INT not null,
  name varchar(140) not null,
  time TIMESTAMP default now(),
  description varchar(2000),
  format varchar(20),
  struct varchar(2000),
  duration varchar(2000),
  language varchar(2000),
  level varchar(2000),
  price integer,
)


insert into course
  (id, teacher_id, name, time)
values(1, 1, 'First course', '2022-10-17 05:40:00');


insert into course
  (id, teacher_id, name, time)
values(2, 1, 'First course', '2022-10-17 05:45:00');