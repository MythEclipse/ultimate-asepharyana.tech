#!/bin/bash

START_SERVER=false

while getopts "s" opt; do
  case $opt in
    s)
      START_SERVER=true
      ;;
    *)
      ;;
  esac
done

if $START_SERVER; then
  bun run dev &
  BUN_PID=$!
  sleep 10
fi

BASE_URL="http://127.0.0.1:4090"

API_ENDPOINTS=(
  "$BASE_URL/api/komik/manga?page=1&order=update"
  "$BASE_URL/api/komik/manhwa?page=1&order=update"
  "$BASE_URL/api/komik/manhua?page=1&order=update"
  "$BASE_URL/api/komik/search?query=naruto"
  "$BASE_URL/api/komik/detail?komik_id=danshi-dato-omotteita-osananajimi-to-no-shinkon-seikatsu-ga-umakuiki-sugiru-kudan-ni-tsuite"
  "$BASE_URL/api/komik/chapter?chapter_url=danshi-dato-omotteita-osananajimi-to-no-shinkon-seikatsu-ga-umakuiki-sugiru-kudan-ni-tsuite-chapter-1-1"
  "$BASE_URL/api/anime"
  "$BASE_URL/api/anime/complete-anime/1"
  "$BASE_URL/api/anime/ongoing-anime/1"
  "$BASE_URL/api/anime/detail/log-horiz-subtitle-indonesia"
  "$BASE_URL/api/anime/full/lgrhzon-episode-1-sub-indo"
  "$BASE_URL/api/anime/search?q=log%20horizon"
  "$BASE_URL/api/anime2"
  "$BASE_URL/api/anime2/complete-anime/1"
  "$BASE_URL/api/anime2/ongoing-anime/1"
  "$BASE_URL/api/anime2/detail/log-horizon"
  "$BASE_URL/api/anime2/search?q=log%20horizon"
  "$BASE_URL/api/uploader"
  "$BASE_URL/api/proxy?url=https://asepharyana.tech"
  "$BASE_URL/api/docs"
)

echo "API Check Report - $(date)"

for url in "${API_ENDPOINTS[@]}"; do
  if [[ "$url" == *"/api/uploader"* ]] || [[ "$url" == *"/api/img-compress"* && "$url" != *"img-compress2"* && "$url" != *"img-compress3"* ]]; then
    echo "$url -> POST (file upload skipped in script)"
  else
    response=$(curl -s -o /dev/null -w "%{http_code} %{redirect_url} %{url_effective}\n" -L "$url")
    echo "$url -> $response"
  fi
done

if $START_SERVER; then
  kill $BUN_PID
  echo "Bun dev server stopped."
fi