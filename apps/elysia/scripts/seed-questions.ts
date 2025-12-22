// Seed script untuk populate Quiz Questions
// Run: bun run apps/elysia/scripts/seed-questions.ts

import { getDb, initializeDb } from '@asepharyana/services';
import { quizQuestions, quizAnswers } from '@asepharyana/services';

// Sample questions data
const sampleQuestions = [
  {
    category: 'Geography',
    difficulty: 'easy',
    text: 'Apa ibu kota Indonesia?',
    answers: ['Jakarta', 'Bandung', 'Surabaya', 'Medan'],
    correctAnswer: 0,
  },
  {
    category: 'Geography',
    difficulty: 'easy',
    text: 'Negara terbesar di dunia adalah?',
    answers: ['China', 'USA', 'Russia', 'Canada'],
    correctAnswer: 2,
  },
  {
    category: 'Geography',
    difficulty: 'medium',
    text: 'Gunung tertinggi di dunia adalah?',
    answers: ['K2', 'Mount Everest', 'Kilimanjaro', 'Mont Blanc'],
    correctAnswer: 1,
  },
  {
    category: 'Science',
    difficulty: 'easy',
    text: 'Planet terdekat dengan Matahari adalah?',
    answers: ['Venus', 'Mars', 'Mercury', 'Earth'],
    correctAnswer: 2,
  },
  {
    category: 'Science',
    difficulty: 'medium',
    text: 'Simbol kimia untuk emas adalah?',
    answers: ['Go', 'Au', 'Gd', 'Ag'],
    correctAnswer: 1,
  },
  {
    category: 'Science',
    difficulty: 'hard',
    text: 'Berapa kecepatan cahaya dalam vakum?',
    answers: ['300,000 km/s', '150,000 km/s', '500,000 km/s', '200,000 km/s'],
    correctAnswer: 0,
  },
  {
    category: 'History',
    difficulty: 'easy',
    text: 'Siapa presiden pertama Indonesia?',
    answers: ['Soekarno', 'Soeharto', 'BJ Habibie', 'Megawati'],
    correctAnswer: 0,
  },
  {
    category: 'History',
    difficulty: 'medium',
    text: 'Perang Dunia II berakhir tahun?',
    answers: ['1943', '1944', '1945', '1946'],
    correctAnswer: 2,
  },
  {
    category: 'History',
    difficulty: 'hard',
    text: 'Siapa yang menemukan mesin cetak?',
    answers: [
      'Johannes Gutenberg',
      'Leonardo da Vinci',
      'Isaac Newton',
      'Galileo Galilei',
    ],
    correctAnswer: 0,
  },
  {
    category: 'Technology',
    difficulty: 'easy',
    text: 'Apa kepanjangan dari CPU?',
    answers: [
      'Central Processing Unit',
      'Computer Personal Unit',
      'Central Program Utility',
      'Computer Processing Unit',
    ],
    correctAnswer: 0,
  },
  {
    category: 'Technology',
    difficulty: 'medium',
    text: 'Siapa pendiri Microsoft?',
    answers: ['Steve Jobs', 'Bill Gates', 'Mark Zuckerberg', 'Elon Musk'],
    correctAnswer: 1,
  },
  {
    category: 'Technology',
    difficulty: 'hard',
    text: 'Bahasa pemrograman apa yang dibuat oleh Guido van Rossum?',
    answers: ['Java', 'Python', 'Ruby', 'JavaScript'],
    correctAnswer: 1,
  },
  {
    category: 'Sports',
    difficulty: 'easy',
    text: 'Berapa jumlah pemain dalam satu tim sepak bola?',
    answers: ['10', '11', '12', '9'],
    correctAnswer: 1,
  },
  {
    category: 'Sports',
    difficulty: 'medium',
    text: "Siapa pemain sepak bola dengan gelar Ballon d'Or terbanyak?",
    answers: [
      'Cristiano Ronaldo',
      'Lionel Messi',
      'Ronaldinho',
      'Zinedine Zidane',
    ],
    correctAnswer: 1,
  },
  {
    category: 'Sports',
    difficulty: 'hard',
    text: 'Negara mana yang menjuarai Piala Dunia FIFA 2018?',
    answers: ['Brazil', 'Germany', 'France', 'Argentina'],
    correctAnswer: 2,
  },
  {
    category: 'Entertainment',
    difficulty: 'easy',
    text: 'Film Disney mana yang menampilkan karakter "Elsa"?',
    answers: ['Moana', 'Frozen', 'Tangled', 'Brave'],
    correctAnswer: 1,
  },
  {
    category: 'Entertainment',
    difficulty: 'medium',
    text: 'Siapa pemeran Iron Man dalam Marvel Cinematic Universe?',
    answers: [
      'Chris Evans',
      'Chris Hemsworth',
      'Robert Downey Jr.',
      'Mark Ruffalo',
    ],
    correctAnswer: 2,
  },
  {
    category: 'Entertainment',
    difficulty: 'hard',
    text: 'Film mana yang memenangkan Oscar Best Picture tahun 2020?',
    answers: ['1917', 'Joker', 'Parasite', 'Once Upon a Time in Hollywood'],
    correctAnswer: 2,
  },
  {
    category: 'Mathematics',
    difficulty: 'easy',
    text: 'Berapa hasil dari 5 + 7?',
    answers: ['11', '12', '13', '14'],
    correctAnswer: 1,
  },
  {
    category: 'Mathematics',
    difficulty: 'medium',
    text: 'Berapa akar kuadrat dari 144?',
    answers: ['10', '11', '12', '13'],
    correctAnswer: 2,
  },
  // ===== GENERAL CATEGORY - EASY (30 questions) =====
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa jumlah hari dalam seminggu?',
    answers: ['5', '6', '7', '8'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Warna apa yang dihasilkan dari campuran merah dan kuning?',
    answers: ['Ungu', 'Hijau', 'Orange', 'Coklat'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Hewan apa yang dikenal sebagai raja hutan?',
    answers: ['Harimau', 'Singa', 'Gajah', 'Beruang'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa jumlah benua di dunia?',
    answers: ['5', '6', '7', '8'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama satelit alami Bumi?',
    answers: ['Mars', 'Venus', 'Bulan', 'Bintang'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa jumlah bulan dalam setahun?',
    answers: ['10', '11', '12', '13'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa warna langit pada siang hari yang cerah?',
    answers: ['Merah', 'Hijau', 'Biru', 'Kuning'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa jumlah jari pada satu tangan?',
    answers: ['3', '4', '5', '6'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama ibukota Indonesia?',
    answers: ['Surabaya', 'Bandung', 'Jakarta', 'Medan'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Hewan apa yang bisa terbang?',
    answers: ['Ikan', 'Burung', 'Kucing', 'Anjing'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa yang digunakan untuk menulis di papan tulis?',
    answers: ['Pensil', 'Kapur', 'Pulpen', 'Spidol'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa hasil 2 + 2?',
    answers: ['3', '4', '5', '6'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama hari setelah Senin?',
    answers: ['Rabu', 'Selasa', 'Kamis', 'Jumat'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Hewan apa yang hidup di air?',
    answers: ['Kucing', 'Burung', 'Ikan', 'Anjing'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa warna daun pada umumnya?',
    answers: ['Merah', 'Kuning', 'Hijau', 'Biru'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa jumlah roda pada sepeda?',
    answers: ['1', '2', '3', '4'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama buah berwarna kuning dan melengkung?',
    answers: ['Apel', 'Jeruk', 'Pisang', 'Anggur'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa yang kita minum setiap hari?',
    answers: ['Minyak', 'Air', 'Cat', 'Bensin'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa jumlah mata manusia?',
    answers: ['1', '2', '3', '4'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama hewan berkaki empat yang menggonggong?',
    answers: ['Kucing', 'Anjing', 'Burung', 'Ikan'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama bentuk yang memiliki 4 sisi sama?',
    answers: ['Segitiga', 'Lingkaran', 'Persegi', 'Oval'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Kapan kita tidur?',
    answers: ['Pagi', 'Siang', 'Sore', 'Malam'],
    correctAnswer: 3,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama musim hujan di Indonesia?',
    answers: ['Musim Panas', 'Musim Dingin', 'Musim Hujan', 'Musim Gugur'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa hasil 10 - 5?',
    answers: ['3', '4', '5', '6'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama planet tempat kita tinggal?',
    answers: ['Mars', 'Venus', 'Bumi', 'Jupiter'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Hewan apa yang menghasilkan susu?',
    answers: ['Ayam', 'Sapi', 'Ikan', 'Burung'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama alat untuk mengukur waktu?',
    answers: ['Penggaris', 'Timbangan', 'Jam', 'Termometer'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Berapa jumlah kaki laba-laba?',
    answers: ['4', '6', '8', '10'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa warna matahari?',
    answers: ['Biru', 'Hijau', 'Kuning', 'Ungu'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'easy',
    text: 'Apa nama makanan pokok orang Indonesia?',
    answers: ['Roti', 'Nasi', 'Mie', 'Kentang'],
    correctAnswer: 1,
  },

  // ===== GENERAL CATEGORY - MEDIUM (30 questions) =====
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Siapa penemu bola lampu?',
    answers: [
      'Albert Einstein',
      'Thomas Edison',
      'Nikola Tesla',
      'Isaac Newton',
    ],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama mata uang Jepang?',
    answers: ['Won', 'Yuan', 'Yen', 'Ringgit'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Negara mana yang terkenal dengan Menara Eiffel?',
    answers: ['Italia', 'Prancis', 'Jerman', 'Spanyol'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Berapa jumlah planet dalam tata surya kita?',
    answers: ['7', '8', '9', '10'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa bahasa resmi Brazil?',
    answers: ['Spanyol', 'Portugis', 'Inggris', 'Prancis'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Siapa presiden pertama Indonesia?',
    answers: ['Soeharto', 'Soekarno', 'Habibie', 'Gus Dur'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama gunung tertinggi di Indonesia?',
    answers: ['Merapi', 'Semeru', 'Jayawijaya', 'Rinjani'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Berapa jumlah provinsi di Indonesia?',
    answers: ['34', '36', '38', '40'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama sungai terpanjang di dunia?',
    answers: ['Amazon', 'Nil', 'Mississipi', 'Yangtze'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Negara mana yang memiliki populasi terbanyak?',
    answers: ['India', 'China', 'USA', 'Indonesia'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama ibu kota Australia?',
    answers: ['Sydney', 'Melbourne', 'Canberra', 'Perth'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Siapa penulis novel Harry Potter?',
    answers: [
      'J.R.R. Tolkien',
      'J.K. Rowling',
      'Stephen King',
      'George R.R. Martin',
    ],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama samudra terluas di dunia?',
    answers: ['Atlantik', 'Hindia', 'Pasifik', 'Arktik'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Berapa lama waktu Bumi mengelilingi Matahari?',
    answers: ['30 hari', '365 hari', '180 hari', '730 hari'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama hewan yang memiliki belalai?',
    answers: ['Jerapah', 'Gajah', 'Badak', 'Kuda Nil'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Negara mana yang terkenal dengan Taj Mahal?',
    answers: ['Pakistan', 'India', 'Bangladesh', 'Nepal'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama vitamin yang didapat dari sinar matahari?',
    answers: ['Vitamin A', 'Vitamin B', 'Vitamin C', 'Vitamin D'],
    correctAnswer: 3,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Siapa penemu telepon?',
    answers: [
      'Thomas Edison',
      'Alexander Graham Bell',
      'Nikola Tesla',
      'Guglielmo Marconi',
    ],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama benua terkecil di dunia?',
    answers: ['Eropa', 'Australia', 'Antartika', 'Afrika'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Berapa derajat sudut siku-siku?',
    answers: ['45°', '90°', '180°', '360°'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama gas yang paling banyak di atmosfer Bumi?',
    answers: ['Oksigen', 'Karbon Dioksida', 'Nitrogen', 'Hidrogen'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Negara mana yang dikenal sebagai Negeri Sakura?',
    answers: ['Korea', 'China', 'Jepang', 'Thailand'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama pulau terbesar di Indonesia?',
    answers: ['Sumatera', 'Jawa', 'Kalimantan', 'Sulawesi'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Siapa pelukis Mona Lisa?',
    answers: [
      'Pablo Picasso',
      'Vincent van Gogh',
      'Leonardo da Vinci',
      'Michelangelo',
    ],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama organ tubuh yang memompa darah?',
    answers: ['Paru-paru', 'Ginjal', 'Jantung', 'Hati'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Berapa suhu di mana air membeku?',
    answers: ['-10°C', '0°C', '10°C', '100°C'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama alat musik gesek yang paling umum?',
    answers: ['Gitar', 'Biola', 'Piano', 'Drum'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Negara mana yang berbentuk sepatu boot?',
    answers: ['Spanyol', 'Prancis', 'Italia', 'Yunani'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Apa nama pahlawan yang dikenal dengan pidato "Jas Merah"?',
    answers: ['Diponegoro', 'Soekarno', 'Kartini', 'Ki Hajar Dewantara'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'medium',
    text: 'Berapa jumlah pemain dalam satu tim sepak bola?',
    answers: ['9', '10', '11', '12'],
    correctAnswer: 2,
  },

  // ===== GENERAL CATEGORY - HARD (30 questions) =====
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa jumlah tulang dalam tubuh manusia dewasa?',
    answers: ['196', '206', '216', '226'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Siapa penemu teori relativitas?',
    answers: [
      'Isaac Newton',
      'Niels Bohr',
      'Albert Einstein',
      'Stephen Hawking',
    ],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama unsur kimia dengan nomor atom 79?',
    answers: ['Perak', 'Emas', 'Platinum', 'Tembaga'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Tahun berapa Indonesia merdeka?',
    answers: ['1944', '1945', '1946', '1947'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama laut terluas di dunia?',
    answers: ['Atlantik', 'Hindia', 'Pasifik', 'Arktik'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa kecepatan cahaya dalam km/detik?',
    answers: ['200.000', '300.000', '400.000', '500.000'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Siapa penemu gravitasi?',
    answers: [
      'Albert Einstein',
      'Isaac Newton',
      'Galileo Galilei',
      'Nikola Tesla',
    ],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama perjanjian kemerdekaan Indonesia?',
    answers: ['Proklamasi', 'Renville', 'Linggarjati', 'Roem-Royen'],
    correctAnswer: 0,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa jumlah kromosom manusia?',
    answers: ['23', '46', '48', '64'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama ibukota baru Indonesia?',
    answers: ['Palangkaraya', 'Nusantara', 'Samarinda', 'Balikpapan'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Siapa pendiri Microsoft?',
    answers: ['Steve Jobs', 'Bill Gates', 'Mark Zuckerberg', 'Jeff Bezos'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama simbol kimia untuk Natrium?',
    answers: ['N', 'Na', 'Nt', 'No'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa pH air murni?',
    answers: ['5', '6', '7', '8'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama planet terbesar di tata surya?',
    answers: ['Saturnus', 'Uranus', 'Jupiter', 'Neptunus'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Siapa presiden AS pertama?',
    answers: [
      'Abraham Lincoln',
      'George Washington',
      'Thomas Jefferson',
      'John Adams',
    ],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama teori asal usul alam semesta yang paling diterima?',
    answers: ['Steady State', 'Big Bang', 'Multiverse', 'String Theory'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa jumlah negara anggota PBB saat ini?',
    answers: ['183', '193', '203', '213'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama enzim yang memecah protein?',
    answers: ['Amilase', 'Lipase', 'Protease', 'Laktase'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Siapa penemu penisilin?',
    answers: [
      'Louis Pasteur',
      'Alexander Fleming',
      'Joseph Lister',
      'Robert Koch',
    ],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama lapisan atmosfer terdekat dengan Bumi?',
    answers: ['Stratosfer', 'Mesosfer', 'Troposfer', 'Termosfer'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa titik didih air dalam Fahrenheit?',
    answers: ['100°F', '180°F', '212°F', '300°F'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama hewan tercepat di darat?',
    answers: ['Singa', 'Cheetah', 'Harimau', 'Kuda'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Siapa penemu DNA?',
    answers: ['Watson & Crick', 'Darwin', 'Mendel', 'Pasteur'],
    correctAnswer: 0,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama kerajaan Islam pertama di Indonesia?',
    answers: ['Demak', 'Samudra Pasai', 'Aceh', 'Banten'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa massa atom Karbon?',
    answers: ['6', '12', '14', '16'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama tokoh yang menulis "Das Kapital"?',
    answers: ['Adam Smith', 'Karl Marx', 'John Keynes', 'Friedrich Engels'],
    correctAnswer: 1,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Siapa pencipta lagu Indonesia Raya?',
    answers: ['W.R. Supratman', 'Ismail Marzuki', 'Gesang', 'Iwan Fals'],
    correctAnswer: 0,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama hormon yang mengatur gula darah?',
    answers: ['Adrenalin', 'Tiroksin', 'Insulin', 'Estrogen'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Berapa lama periode revolusi Bulan?',
    answers: ['7 hari', '14 hari', '27 hari', '30 hari'],
    correctAnswer: 2,
  },
  {
    category: 'general',
    difficulty: 'hard',
    text: 'Apa nama efek rumah kaca utama?',
    answers: ['Oksigen', 'Nitrogen', 'CO2', 'Helium'],
    correctAnswer: 2,
  },
];

async function seedQuestions() {
  try {
    // Initialize database
    const databaseUrl = process.env.DATABASE_URL;
    if (!databaseUrl) {
      throw new Error('DATABASE_URL not found in environment variables');
    }

    initializeDb(databaseUrl);
    const db = getDb();

    console.log('🌱 Starting to seed questions...');

    let totalQuestions = 0;
    let totalAnswers = 0;

    for (const question of sampleQuestions) {
      // Generate unique ID
      const questionId = `q_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;

      // Insert question
      await db.insert(quizQuestions).values({
        id: questionId,
        text: question.text,
        category: question.category,
        difficulty: question.difficulty,
        correctAnswer: question.correctAnswer,
      });

      totalQuestions++;
      console.log(`✅ Added question: ${question.text}`);

      // Insert answers
      for (let i = 0; i < question.answers.length; i++) {
        const answerId = `a_${questionId}_${i}`;

        await db.insert(quizAnswers).values({
          id: answerId,
          questionId: questionId,
          text: question.answers[i],
          answerIndex: i,
        });

        totalAnswers++;
      }
    }

    console.log('\n🎉 Seeding completed!');
    console.log(`📊 Total questions added: ${totalQuestions}`);
    console.log(`📊 Total answers added: ${totalAnswers}`);

    // Show statistics
    console.log('\n📈 Questions by category:');
    const categories = sampleQuestions.reduce(
      (acc, q) => {
        acc[q.category] = (acc[q.category] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>,
    );

    Object.entries(categories).forEach(([category, count]) => {
      console.log(`   ${category}: ${count} questions`);
    });

    console.log('\n📈 Questions by difficulty:');
    const difficulties = sampleQuestions.reduce(
      (acc, q) => {
        acc[q.difficulty] = (acc[q.difficulty] || 0) + 1;
        return acc;
      },
      {} as Record<string, number>,
    );

    Object.entries(difficulties).forEach(([difficulty, count]) => {
      console.log(`   ${difficulty}: ${count} questions`);
    });
  } catch (error) {
    console.error('❌ Error seeding questions:', error);
    throw error;
  }
}

// Run the seed function
seedQuestions()
  .then(() => {
    console.log('\n✨ Seed script completed successfully!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n💥 Seed script failed:', error);
    process.exit(1);
  });
