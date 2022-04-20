: ?DAY 32 < IF ." Looks good " ELSE ." no way " THEN ;
32 ?DAY
22 ?DAY

: /CHECK DUP 0= IF ." invalid " DROP ELSE / THEN ;
32 6 /CHECK
.
34 0 /CHECK

: EGGSIZE DUP 18 < IF  ." reject "      ELSE DUP 21 < IF  ." small "       ELSE DUP 24 < IF  ." medium "      ELSE DUP 27 < IF  ." large "       ELSE DUP 30 < IF  ." extra large " ELSE   ." error " THEN THEN THEN THEN THEN DROP ;

20 EGGSIZE
