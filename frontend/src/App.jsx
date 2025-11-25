import React, {useState} from 'react';
import {piAuthenticate, piCreatePayment} from './services/pi-sdk';

export default function App(){
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(false);

  async function handleLogin(){
    setLoading(true);
    try {
      const auth = await piAuthenticate();
      setUser(auth.user || auth); // contoh
    } catch(err){
      console.error(err);
      alert('Login Pi gagal: ' + err.message);
    } finally { setLoading(false); }
  }

  async function handleBuy(){
    if(!user){ alert('Login dulu'); return; }
    setLoading(true);
    try {
      // 1) create PO on backend -> get merchantRef
      const poResp = await fetch('/api/purchase_orders', {
        method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify({item:'Premium',amountPi:1.5})
      });
      const po = await poResp.json();
      // 2) call Pi SDK to create payment (A2U)
      const pay = await piCreatePayment({amountPi: po.amountPi, merchantRef: po.id, metadata:{memo: 'Pembelian Premium'}});
      // 3) optionally poll backend for PO update / wait for webhook
      alert('Payment requested, id: ' + pay.paymentId);
    } catch(e){ console.error(e); alert(e.message); }
    finally { setLoading(false); }
  }

  return (
    <div style={{padding:20}}>
      <h1>Pi Merchant Demo</h1>
      {!user ? 
        <button onClick={handleLogin} disabled={loading}>Login via Pi Browser</button>
        :
        <>
          <div>Halo, {user.username || user.displayName}</div>
          <button onClick={handleBuy} disabled={loading}>Beli 1.5 PI - Premium</button>
        </>
      }
    </div>
  );
}
