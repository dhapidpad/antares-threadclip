const fs = require("fs");

const posts = [];
for (let i = 1; i <= 20; i++) {
  const replies = [];
  const pId = `post_${i}`;
  
  // Level 1 replies
  for (let j = 1; j <= 3; j++) {
    const r1Id = `r_${i}_${j}`;
    replies.push({
      id: r1Id,
      replied_to_id: pId,
      text: `Bình luận cấp 1 số ${j} về chủ đề video short AI`,
      author: `user_${j}`,
      created_at: 1700000000 + i * 1000 + j * 100,
      reply_count: 2,
      like_count: j * 5
    });

    // Level 2 replies
    for (let k = 1; k <= 2; k++) {
      const r2Id = `r_${i}_${j}_${k}`;
      replies.push({
        id: r2Id,
        replied_to_id: r1Id,
        text: `Phản hồi cấp 2 từ tác giả giải thích chi tiết hơn`,
        author: (k === 1) ? "creator_vn" : `user_sub_${k}`,
        created_at: 1700000000 + i * 1000 + j * 100 + k * 20,
        reply_count: 1,
        like_count: 3
      });

      // Level 3 replies
      const r3Id = `r_${i}_${j}_${k}_1`;
      replies.push({
        id: r3Id,
        replied_to_id: r2Id,
        text: `Thắc mắc cấp 3 về tính năng TTS và frame rate`,
        author: `viewer_deep_${k}`,
        created_at: 1700000000 + i * 1000 + j * 100 + k * 20 + 5,
        reply_count: 1,
        like_count: 1
      });

      // Level 4 replies
      const r4Id = `r_${i}_${j}_${k}_1_1`;
      replies.push({
        id: r4Id,
        replied_to_id: r3Id,
        text: `Phản hồi cấp 4 xác nhận tính chính xác từng khung hình`,
        author: "creator_vn",
        created_at: 1700000000 + i * 1000 + j * 100 + k * 20 + 8,
        reply_count: 0,
        like_count: 8
      });
    }
  }

  posts.push({
    id: pId,
    text: `Bài viết số ${i}: Hướng dẫn dựng video ngắn từ Threads với Antares Studio local-first!`,
    author: "creator_vn",
    created_at: 1700000000 + i * 1000,
    permalink: `https://threads.net/@creator_vn/post/${pId}`,
    reply_count: replies.length,
    like_count: 25 * i,
    replies: replies
  });
}

const data = {
  account: {
    id: "creator_vn_id",
    username: "creator_vn"
  },
  posts: posts
};

fs.mkdirSync("testdata", { recursive: true });
fs.writeFileSync("testdata/threads_sample.json", JSON.stringify(data, null, 2));
console.log(`Generated ${posts.length} posts with ${posts.reduce((acc, p) => acc + p.replies.length, 0)} replies.`);
