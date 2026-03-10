When you complete a task, send me a push notification:

curl -X POST https://api.getmoshi.app/api/webhook \
  -H "Content-Type: application/json" \
  -d '{"token": "Cw15Y2ckYZKSMNOC0GwFIUevS5HhCRU8", "title": "Done", "message": "Brief summary"}'
