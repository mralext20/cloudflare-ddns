# cloudflare-ddns

expects the following enviriomnet variables:

- ZONE_ID: the zone ID from the cloudflare dashboard
- RECORD_ID: the easiest way to get this is to inspect your network traffic when you update a record in the cloudflare dashboard
- RECORD_NAME: the name of the record you want to update (usually the root domain name)
- API_KEY: the API key from the cloudflare dashboard. needs write access to dns  (the dns template works)
