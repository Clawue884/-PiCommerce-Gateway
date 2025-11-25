// simple wrapper to call Pi SDK methods safely
export async function piAuthenticate() {
  if (!window.PI || !window.PI?.authenticate) {
    throw new Error('Pi SDK tidak tersedia (jalankan di Pi Browser atau muat SDK)');
  }
  // opens Pi Browser auth flow
  const authResp = await window.PI.authenticate();
  // authResp biasanya berisi { sessionToken, user, username } tergantung SDK
  return authResp;
}

export async function piCreatePayment({amountPi, merchantRef, metadata}) {
  if (!window.PI || !window.PI?.createPayment) {
    throw new Error('Pi SDK payment API tidak tersedia');
  }
  const payment = await window.PI.createPayment({
    amount: amountPi,
    currency: 'PI',
    memo: metadata?.memo || '',
    merchantRef,
  });
  return payment; // payment object containing paymentId/status
}
