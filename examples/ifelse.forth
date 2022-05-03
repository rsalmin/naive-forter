: ?DAY 32 < IF ." Looks good " ELSE ." no way " THEN ;
32 ?DAY
22 ?DAY

: /CHECK DUP 0= IF ." invalid " DROP ELSE / THEN ;
32 6 /CHECK
.
34 0 /CHECK

: EGGSIZE
    DUP 18 < IF  ." reject "      ELSE
    DUP 21 < IF  ." small "       ELSE
    DUP 24 < IF  ." medium "      ELSE
    DUP 27 < IF  ." large "       ELSE
    DUP 30 < IF  ." extra large " ELSE
        ." error "
    THEN THEN THEN THEN THEN DROP ;

20 EGGSIZE

: BOXTEST ( length width height -- )
    6 > ROT 22 > ROT 19 > AND AND IF ." Big enough " THEN ;

23 20 7 BOXTEST


: /CHECK DUP 0= ABORT" zero denominator " / ;
8 2 /CHECK
.
8 0 /CHECK

