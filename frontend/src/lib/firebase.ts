// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseConfig = {
    apiKey: "AIzaSyDKJM5cnYy_2ggiZ0ZM2VLtKzgGYl6iTUU",
    authDomain: "structure2schematic.firebaseapp.com",
    projectId: "structure2schematic",
    storageBucket: "structure2schematic.firebasestorage.app",
    messagingSenderId: "698545743342",
    appId: "1:698545743342:web:d0f7b30b387927e6d39aff",
    measurementId: "G-F7B191F8XK"
};

// Initialize Firebase
export const app = initializeApp(firebaseConfig);
export const analytics = getAnalytics(app);