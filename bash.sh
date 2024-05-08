#!/bin/bash
cd /usr/local/bin/buffalo_trace_scraping
./scraper
sleep 5
./send_email
