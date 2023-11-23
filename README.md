# cloudflared

expects the following enviriomnet variables:

- ZONE_ID: the zone ID from the cloudflare dashboard
- RECORD_ID: the easiest way to get this is to inspect your network traffic when you update a record in the cloudflare dashboard
- API_KEY: the API key from the cloudflare dashboard. needs write access to dns  (the dns template works)

