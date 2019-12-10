#!/usr/bin/env bash

GST_DEBUG=4 gst-launch-1.0 pulsesrc ! mulawenc ! rtppcmupay ! udpsink port=6666 \
    udpsrc caps="application/x-rtp" port=6666 ! rtppcmudepay ! mulawdec ! autoaudiosink
