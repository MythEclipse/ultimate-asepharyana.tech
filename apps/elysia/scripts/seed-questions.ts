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
  // ===== GENERAL CATEGORY =====
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
