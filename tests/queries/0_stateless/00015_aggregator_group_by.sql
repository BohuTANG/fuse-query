SELECT max(number) FROM numbers_mt (10) where number > 99999999998 GROUP BY number%3;

SELECT 'SELECT avg(number), max(number+1)+1 FROM numbers_mt(10000) where number > 2 GROUP BY 1';
SELECT avg(number), max(number+1)+1 FROM numbers_mt(10000) where number > 2 GROUP BY 1;

SELECT 'SELECT number%3 as c1, number%2 as c2 FROM numbers_mt(10000) where number > 2 group by c1, c2 order by c1,c2';
SELECT number%3 as c1, number%2 as c2 FROM numbers_mt(10000) where number > 2 group by c1, c2 order by c1,c2;
