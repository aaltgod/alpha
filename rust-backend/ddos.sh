while true;
do
    curl -s -o /dev/null -w "%{http_code}"-X POST http://localhost:2137/create-service -H "accept: application/json" -H "Content-Type: application/json" -d '{"name": "kekew", "port": 2137, "flag_regexp":"[A-Za-z0-9]{31}="}';
    printf ", ";
#    sleep 1s;
done
