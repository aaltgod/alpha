while true;
do
    curl -s -o /dev/null -w "%{http_code}"-X POST http://localhost:2137/upsert-service -H "accept: application/json" -H "Content-Type: application/json" -d '{
    "service": {
        "name": "СЕРВИСА",
        "port": 2137
    },
    "rule_ids": [1, 2, 3, 4, 5]
}';
    printf ", ";
#    sleep 1s;
done
