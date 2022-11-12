do
$do$
    begin
        for i in 1..100
            loop
                insert into "user" (affiliation_id, name, surname, birth_date)
                values (1, 'John', 'Doe', now()::timestamp - interval '20 years');

                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2018-11-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2018-11-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2018-11-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2018-11-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2018-11-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 13000, '2018-12-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2018-12-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2018-12-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2018-12-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2018-12-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2018-12-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2019-01-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2019-01-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2019-01-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2019-01-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2019-01-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2019-01-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 19500, '2019-02-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2019-02-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2019-02-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2019-02-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 19500, '2019-02-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2019-02-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2019-03-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2019-03-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2019-03-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2019-03-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2019-03-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2019-03-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2019-04-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2019-04-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2019-04-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2019-04-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2019-04-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2019-04-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2019-05-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2019-05-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2019-05-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2019-05-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2019-05-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2019-05-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2019-06-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2019-06-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2019-06-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2019-06-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2019-06-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2019-06-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2019-07-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2019-07-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2019-07-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2019-07-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2019-07-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2019-07-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2019-08-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2019-08-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2019-08-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2019-08-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2019-08-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2019-08-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2019-08-31 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2019-09-05 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2019-09-10 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2019-09-15 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2019-09-20 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2019-09-25 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2019-09-30 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2019-10-05 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2019-10-10 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 13000, '2019-10-15 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2019-10-20 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2019-10-25 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2019-10-30 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2019-11-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2019-11-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2019-11-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2019-11-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2019-11-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2019-11-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2019-12-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2019-12-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2019-12-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2019-12-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2019-12-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2019-12-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2020-01-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2020-01-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2020-01-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2020-01-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2020-01-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2020-01-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2020-02-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2020-02-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 13000, '2020-02-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2020-02-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2020-02-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2020-02-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2020-03-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2020-03-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2020-03-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2020-03-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2020-03-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 13000, '2020-03-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2020-04-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2020-04-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2020-04-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2020-04-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2020-04-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2020-04-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2020-05-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2020-05-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2020-05-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2020-05-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2020-05-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2020-05-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2020-06-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2020-06-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2020-06-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2020-06-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2020-06-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2020-06-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2020-07-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2020-07-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2020-07-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2020-07-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2020-07-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2020-07-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2020-07-31 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2020-08-05 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2020-08-10 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2020-08-15 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2020-08-20 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2020-08-25 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2020-08-30 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2020-09-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2020-09-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2020-09-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2020-09-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2020-09-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2020-09-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2020-10-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2020-10-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2020-10-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2020-10-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2020-10-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2020-10-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2020-11-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2020-11-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2020-11-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2020-11-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2020-11-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2020-11-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2020-12-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2020-12-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2020-12-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2020-12-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2020-12-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2020-12-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2021-01-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2021-01-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2021-01-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2021-01-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2021-01-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2021-01-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2021-02-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2021-02-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2021-02-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2021-02-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2021-02-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 13000, '2021-02-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2021-03-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2021-03-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2021-03-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2021-03-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2021-03-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2021-03-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2021-04-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2021-04-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2021-04-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2021-04-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2021-04-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2021-04-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 19500, '2021-05-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2021-05-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2021-05-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2021-05-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2021-05-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2021-05-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2021-06-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2021-06-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2021-06-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2021-06-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2021-06-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2021-06-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2021-07-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2021-07-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2021-07-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2021-07-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 13000, '2021-07-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2021-07-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2021-07-31 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2021-08-05 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 19500, '2021-08-10 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2021-08-15 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2021-08-20 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2021-08-25 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2021-08-30 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2021-09-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2021-09-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2021-09-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2021-09-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2021-09-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2021-09-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2021-10-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2021-10-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2021-10-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2021-10-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2021-10-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2021-10-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2021-11-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2021-11-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2021-11-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2021-11-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2021-11-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2021-11-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 13000, '2021-12-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2021-12-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2021-12-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2021-12-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2021-12-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2021-12-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2022-01-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2022-01-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2022-01-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2022-01-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2022-01-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2022-01-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2022-02-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2022-02-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2022-02-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2022-02-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2022-02-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2022-02-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2022-03-03 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2022-03-08 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2022-03-13 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2022-03-18 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2022-03-23 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2022-03-28 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2022-04-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2022-04-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 26000, '2022-04-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2022-04-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 13000, '2022-04-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2022-04-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 13000, '2022-05-02 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 19500, '2022-05-07 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 19500, '2022-05-12 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 19500, '2022-05-17 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 26000, '2022-05-22 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 19500, '2022-05-27 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2022-06-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 26000, '2022-06-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2022-06-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2022-06-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2022-06-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 13000, '2022-06-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 13000, '2022-07-01 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 19500, '2022-07-06 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2022-07-11 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2022-07-16 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2022-07-21 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 19500, '2022-07-26 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 19500, '2022-07-31 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'advance', 13000, '2022-08-05 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 26000, '2022-08-10 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2022-08-15 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2022-08-20 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2022-08-25 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 19500, '2022-08-30 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 13000, '2022-09-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'advance', 26000, '2022-09-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2022-09-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'advance', 13000, '2022-09-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'vacation', 26000, '2022-09-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 150000, 'salary', 26000, '2022-09-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2022-10-04 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'salary', 13000, '2022-10-09 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 26000, '2022-10-14 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 26000, '2022-10-19 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'vacation', 19500, '2022-10-24 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 200000, 'salary', 19500, '2022-10-29 22:58:05.424761');
                INSERT INTO public.accounting (user_id, amount, payment_type, tax_charged, date_paid)
                VALUES (i, 100000, 'vacation', 19500, '2022-11-03 22:58:05.424761');
            end loop;
    end
$do$