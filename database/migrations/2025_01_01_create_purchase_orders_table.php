<?php
use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

class CreatePurchaseOrdersTable extends Migration {
  public function up() {
    Schema::create('purchase_orders', function (Blueprint $table) {
      $table->id();
      $table->string('merchant_ref')->unique();
      $table->string('status')->default('created'); // created, pending_payment, paid, settled, cancelled
      $table->unsignedDecimal('amount_pi', 18, 8);
      $table->json('metadata')->nullable();
      $table->string('pi_payment_id')->nullable();
      $table->timestamps();
    });
  }
  public function down(){ Schema::dropIfExists('purchase_orders'); }
}
