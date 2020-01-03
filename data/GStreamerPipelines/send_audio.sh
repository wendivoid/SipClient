#!/usr/bin/env bash

GST_DEBUG=4 gst-launch-1.0 pulsesrc ! mulawenc ! rtppcmupay ! udpsink port=21066
