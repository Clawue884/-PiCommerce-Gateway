# PiCommerce Gateway — Troubleshooting Guide

This document covers common issues developers may encounter when setting up or integrating PiCommerce Gateway, along with suggested solutions.

---

## Pi SDK Not Available

**Cause**
- Application is not running inside the Pi Browser
- Pi JavaScript SDK is not loaded

**Solution**
- Open the application using **Pi Browser**
- Ensure the Pi SDK script is loaded before calling any Pi SDK methods
- Do not attempt to authenticate or create payments in a regular browser

---

## Payment Creation Fails in Frontend

**Symptoms**
- `window.PI.createPayment` is undefined
- Payment popup does not appear

**Cause**
- Pi SDK not initialized
- Incorrect runtime environment

**Solution**
- Confirm the app is running inside Pi Browser
- Verify Pi SDK is available before calling payment functions

---

## Webhook Signature Validation Failed

**Symptoms**
- Webhook endpoint returns `400 invalid sig`
- Order status is not updated after payment

**Cause**
- Incorrect webhook secret
- Payload modified before verification
- Missing or incorrect `X-Pi-Signature` header

**Solution**
- Ensure the webhook secret matches the value configured in the Pi Developer Portal
- Do not modify the request body before verifying the signature
- Confirm the `X-Pi-Signature` header is present

---

## Purchase Order Not Updated After Payment

**Symptoms**
- Payment succeeds in Pi
- Order status remains `created`

**Possible Causes**
- Webhook request not received
- `merchantRef` does not match database record
- Webhook endpoint not publicly accessible

**Solution**
- Verify the webhook URL is reachable from the internet
- Check application logs for incoming webhook requests
- Ensure the frontend uses the exact `merchant_ref` returned by the backend

---

## PurchaseOrder Model Not Found

**Observation**
- Controller references `App\Models\PurchaseOrder`
- Model file is not present in the repository

**Explanation**
- The project is currently in **alpha**
- Database schema is defined via migrations
- Model implementation may be added in future updates

**Recommendation**
- Use the migration file as the current source of truth
- Implement the model if extending backend functionality

---

## Database Migration Issues

**Cause**
- The migration has already been applied
- The database contains existing tables from a previous run

**Solution**
```bash
php artisan migrate:fresh

⚠️ Warning:
This command will drop all existing tables and data before re-running migrations. Use it only in development, not in production.

Tip
Check the migrations table in the database to see which migrations have already been executed.

## Webhook Not Triggered During Local Development

**Symptoms**
- Payment completes successfully in Pi
- Backend order status is not updated
- No webhook logs appear in the application

**Cause**
- The Pi Platform cannot reach a local development server
- Localhost (`127.0.0.1` / `localhost`) is not publicly accessible

**Solution**
- Use a tunneling tool (such as **ngrok**) to expose your local server
- Configure the public tunnel URL as the webhook endpoint in the Pi Developer Portal
- Ensure the `/webhook/pi` endpoint is reachable over HTTPS

**Checklist**
- Backend server is running
- Tunnel URL is active and correct
- Webhook endpoint returns `200 OK`
- Webhook signature header is present


Debugging Tips

Enable Laravel logging
Inspect logs in storage/logs/laravel.log
Log webhook payloads during development
Test payment flows inside Pi Browser only

Getting Help

If issues persist:
Open a GitHub issue with relevant logs
Do not include secrets or private keys
Provide clear reproduction steps
Contributions to improve this guide are welcome.