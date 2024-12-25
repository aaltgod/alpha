while true;
do
    curl -s -o /dev/null -w "%{http_code}"-X POST http://127.0.0.1:5001/echo -H "accept: application/json" -H "Content-Type: application/json" -d '{
    "service": {
        "name": "EQWEQWEQWEEQWEQWEQWE23EWEQWEQWE=",
        "port": 2137
    },
    "rule_ids": [1, 2, 3123, 1, 12125] 
}';
    printf ", ";
#    sleep 1s;
done
