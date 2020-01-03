#!/usr/bin/env bash

GST_DEBUG_DUMP_DOT_DIR=. gst-launch-1.0 -e \
    udpsrc caps="application/x-rtp,media=(string)audio,clock-rate=(int)8000,payload=(int)0" \
    port=5061 ! rtppcmudepay ! mulawdec ! queue ! fakesink dump=true\
    autoaudiosrc ! mulawenc ! rtppcmupay ! udpsink host=192.168.1.76 port=24968 \
