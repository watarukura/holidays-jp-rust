# holidays-jp-rust

## spec

```bash
#!/bin/bash

specify_date=$1
today=$(date '+%Y-%m-%d' --date "${specify_date:-today}")
tomorrow=$(date '+%Y-%m-%d' --date "tomorrow ${specify_date}")
is_holiday=$(curl -sL https://holidays-jp.github.io/api/v1/date.json \
  | jq -r '. | keys | .[]' \
  | cat torana-holidays - \
  | grep -E -m 1 "${today}|${tomorrow}")
if [[ "${is_holiday}" != "" ]]; then
  echo "holiday"
  exit 1
fi

week=$(date +%u --date "${specify_date:-today}")
if [[ "${week}" -ge 5 ]]; then
  echo "weekend"
  exit 1
fi

exit 0
```

## Thanks

- https://national-holidays.jp/
- https://holidays-jp.github.io/