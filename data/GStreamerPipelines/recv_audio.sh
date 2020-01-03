#!/usr/bin/env bash

GST_DEBUG=4 gst-launch-1.0 udpsrc caps="application/x-rtp,media=(string)audio,clock-rate=(int)8000,payload=(int)0" \
    port=5061 ! rtppcmudepay ! mulawdec ! audioconvert ! audioresample ! pulsesink
