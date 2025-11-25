<?php
namespace App\Http\Controllers;
use Illuminate\Http\Request;
use App\Models\PurchaseOrder;
use Illuminate\Support\Str;
use Illuminate\Support\Facades\Log;

class PurchaseOrderController extends Controller {
  public function store(Request $r){
    $amount = $r->input('amountPi');
    $po = PurchaseOrder::create([
      'merchant_ref' => 'PO-'.Str::upper(Str::random(10)),
      'amount_pi' => $amount,
      'metadata' => $r->input('metadata', []),
      'status' => 'created'
    ]);
    return response()->json($po,201);
  }

  // webhook from Pi SDK/backend: verify signature HMAC + timestamp
  public function webhook(Request $r){
    $sig = $r->header('X-Pi-Signature');
    $payload = $r->getContent();
    $secret = config('services.pi.webhook_secret');
    if (!$this->verifySignature($payload, $sig, $secret)) {
      Log::warning('Invalid webhook signature');
      return response('invalid sig', 400);
    }
    $data = $r->json()->all();
    // example: {event: 'payment.completed', paymentId: '...', merchantRef: 'PO-XXXX', status:'paid'}
    if ($data['event'] === 'payment.completed') {
      $po = PurchaseOrder::where('merchant_ref', $data['merchantRef'])->first();
      if ($po) {
        $po->status = 'paid';
        $po->pi_payment_id = $data['paymentId'];
        $po->save();
      }
    }
    return response('ok',200);
  }

  private function verifySignature($payload, $sig, $secret){
    if(!$sig || !$secret) return false;
    $computed = hash_hmac('sha256', $payload, $secret);
    return hash_equals($computed, $sig);
  }
}
